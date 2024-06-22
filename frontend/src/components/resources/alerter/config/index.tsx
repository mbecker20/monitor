import { Config } from "@components/config";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { useState } from "react";
import { EndpointConfig } from "./endpoint";
import { AlertTypeConfig } from "./alert_types";
import { ResourcesConfig } from "./resources";

export const AlerterConfig = ({ id }: { id: string }) => {
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Alerter", id },
  }).data;
  const config = useRead("GetAlerter", { alerter: id }).data?.config;
  const global_disabled =
    useRead("GetCoreInfo", {}).data?.ui_write_disabled ?? false;
  const { mutateAsync } = useWrite("UpdateAlerter");
  const [update, set] = useState<Partial<Types.AlerterConfig>>({});

  if (!config) return null;
  const disabled = global_disabled || perms !== Types.PermissionLevel.Write;

  return (
    <Config
      disabled={disabled}
      config={config}
      update={update}
      set={set}
      onSave={async () => {
        await mutateAsync({ id, config: update });
      }}
      components={{
        general: [
          {
            label: "General",
            components: {
              enabled: true,
              endpoint: (endpoint, set) => (
                <EndpointConfig
                  endpoint={endpoint!}
                  set={(endpoint) => set({ endpoint })}
                  disabled={disabled}
                />
              ),
              alert_types: (alert_types, set) => (
                <AlertTypeConfig
                  alert_types={alert_types!}
                  set={(alert_types) => set({ alert_types })}
                  disabled={disabled}
                />
              ),
              resources: (resources, set) => (
                <ResourcesConfig
                  resources={resources!}
                  set={(resources) => set({ resources })}
                  disabled={disabled}
                  blacklist={false}
                />
              ),
              except_resources: (resources, set) => (
                <ResourcesConfig
                  resources={resources!}
                  set={(except_resources) => set({ except_resources })}
                  disabled={disabled}
                  blacklist={true}
                />
              ),
            },
          },
        ],
      }}
    />
  );
};
