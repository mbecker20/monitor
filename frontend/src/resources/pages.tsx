import { Factory, Hammer, Rocket, Server } from "lucide-react";
import { useRead } from "@hooks";
import { Resources } from "@layouts/resources";
import { DeploymentCard } from "./deployment/card";
import { BuildCard } from "./build/card";
import { ServerCard } from "./server/card";
import { BuilderCard } from "./builder/card";

const DeploymentsSummary = () => {
  const summary = useRead("GetDeploymentsSummary", {}).data;
  if (!summary) return <>...</>;
  else {
    const { total, running, stopped, unknown } = summary;
    return (
      <>
        {total} Total, {running} Running, {stopped} Stopped, {unknown} Unknown
      </>
    );
  }
};

export const Deployments = () => {
  const deployments = useRead("ListDeployments", {}).data;
  return (
    <Resources
      type="Deployment"
      summary={<DeploymentsSummary />}
      icon={<Rocket className="w-4 h-4" />}
      components={(search) => (
        <>
          {deployments
            ?.filter((d) => d.name.includes(search) || search.includes(d.name))
            .map(({ id }) => (
              <DeploymentCard key={id} id={id} />
            ))}
        </>
      )}
    />
  );
};

const ServersSummary = () => {
  const summary = useRead("GetServersSummary", {}).data;
  if (!summary) return <>...</>;
  else {
    const { total, healthy, unhealthy } = summary;
    return (
      <>
        {total} Total, {healthy} Healthy, {unhealthy} Unhealthy
      </>
    );
  }
};

export const Servers = () => {
  const servers = useRead("ListServers", {}).data;
  return (
    <Resources
      type="Server"
      summary={<ServersSummary />}
      icon={<Server className="w-4 h-4" />}
      components={(search) => (
        <>
          {servers
            ?.filter((d) => d.name.includes(search) || search.includes(d.name))
            .map(({ id }) => (
              <ServerCard key={id} id={id} />
            ))}
        </>
      )}
    />
  );
};

export const Builds = () => {
  const builds = useRead("ListBuilds", {}).data;
  const summary = useRead("GetBuildsSummary", {}).data;

  return (
    <Resources
      type="Build"
      summary={summary ? `${summary?.total} Total` : "..."}
      icon={<Hammer className="w-4 h-4" />}
      components={(search) => (
        <>
          {builds
            ?.filter((d) => d.name.includes(search) || search.includes(d.name))
            .map(({ id }) => (
              <BuildCard key={id} id={id} />
            ))}
        </>
      )}
    />
  );
};

export const Builders = () => {
  const builders = useRead("ListBuilders", {}).data;
  const summary = useRead("GetBuildersSummary", {}).data;

  return (
    <Resources
      type="Builder"
      summary={summary ? `${summary?.total} Total` : "..."}
      icon={<Factory className="w-4 h-4" />}
      components={(search) => (
        <>
          {builders
            ?.filter((d) => d.name.includes(search) || search.includes(d.name))
            .map(({ _id }) => (
              <BuilderCard key={_id?.$oid} id={_id?.$oid!} />
            ))}
        </>
      )}
    />
  );
};