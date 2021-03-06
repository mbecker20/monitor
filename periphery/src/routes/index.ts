import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import container from "./container";
import deploy from "./deploy";
import git from "./git";
import networks from "./networks";
import pm2 from "./pm2";
import server from "./server";
import status from "./status";

const routes = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app
    .register(git)
    .register(deploy)
    .register(container)
    .register(server)
    .register(status)
    .register(networks)
    .register(pm2);

  done();
});

export default routes;
