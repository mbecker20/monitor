import { ResourceUpdates } from "@components/updates/resource";
import { useSetRecentlyViewed } from "@hooks";
import { Resource } from "@layouts/resource";
import { ServerConfig } from "@resources/server/config";
import { ServerStats } from "@resources/server/stats";
import {
  ServerName,
  ServerStatusIcon,
  ServerSpecs,
} from "@resources/server/util";
import { CardDescription } from "@ui/card";
import { useParams } from "react-router-dom";

export const ServerPage = () => {
  const id = useParams().serverId;
  const push = useSetRecentlyViewed();

  if (!id) return null;
  push("Server", id);

  return (
    <Resource
      title={<ServerName serverId={id} />}
      info={
        <div className="flex items-center gap-4">
          <ServerStatusIcon serverId={id} />
          <CardDescription className="hidden md:block">|</CardDescription>
          <ServerSpecs server_id={id} />
        </div>
      }
      actions={null}
    >
      <ResourceUpdates type="Server" id={id} />
      <ServerStats />
      <ServerConfig />
    </Resource>
  );
};