import { User } from "@monitor/types";
import { mergeCommandLogError, PULL_DEPLOYMENT } from "@monitor/util";
import { execute, pull } from "@monitor/util-node";
import { FastifyInstance } from "fastify";
import { join } from "path";
import { WebSocket } from "ws";
import { DEPLOYMENT_REPO_PATH, PERMISSIONS_DENY_LOG, SYSTEM_OPERATOR } from "../../config";
import { sendAlert } from "../../util/helpers";
import { pullPeriphery } from "../../util/periphery/git";
import { addDeploymentUpdate } from "../../util/updates";

async function pullDeploymentRepo(
  app: FastifyInstance,
  client: WebSocket,
  user: User,
  { deploymentID }: { deploymentID: string }
) {
  if (app.deployActionStates.busy(deploymentID)) {
    sendAlert(client, "bad", "deployment busy, try again in a bit");
    return;
  }
  const deployment = await app.deployments.findById(deploymentID);
  if (!deployment) {
    return;
  }
  if (user.permissions! < 2 && !deployment.owners.includes(user.username)) {
    addDeploymentUpdate(
      app,
      deploymentID,
      PULL_DEPLOYMENT,
      "Pull Deployemnt (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      undefined,
      true
    );
    return;
  }
  const { branch, containerName, onPull, serverID } = deployment;
  const server = await app.servers.findById(serverID!);
  if (!server) {
    return;
  }
  app.deployActionStates.set(deploymentID, "pulling", true);
  app.broadcast(
    PULL_DEPLOYMENT,
    { deploymentID, complete: false },
    app.deploymentUserFilter(deploymentID)
  );
  if (server.isCore) {
    const pullCle = await pull(
      join(DEPLOYMENT_REPO_PATH, containerName!),
      branch
    );
    const onPullCle =
      onPull &&
      (await execute(
        `cd ${join(
          DEPLOYMENT_REPO_PATH,
          containerName!,
          onPull.path || ""
        )} && ${onPull.command}`
      ));
    const { command, log, isError } = mergeCommandLogError(
      { name: "pull", cle: pullCle },
      { name: "on pull", cle: onPullCle }
    );
    addDeploymentUpdate(
      app,
      deploymentID,
      PULL_DEPLOYMENT,
      command,
      log,
      SYSTEM_OPERATOR,
      "",
      isError
    );
  } else {
    const { command, log, isError } = await pullPeriphery(server, deployment);
    await addDeploymentUpdate(
      app,
      deploymentID,
      PULL_DEPLOYMENT,
      command,
      log,
      SYSTEM_OPERATOR,
      "",
      isError
    );
  }
  app.broadcast(
    PULL_DEPLOYMENT,
    { deploymentID, complete: true },
    app.deploymentUserFilter(deploymentID)
  );
  app.deployActionStates.set(deploymentID, "pulling", false);
}

export default pullDeploymentRepo;