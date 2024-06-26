import { ActionWithDialog } from "@components/util";
import { useExecute, useRead } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { Card, CardHeader } from "@ui/card";
import { Route } from "lucide-react";
import { ProcedureConfig } from "./config";
import { ProcedureTable } from "./table";
import { DeleteResource, NewResource } from "../common";
import {
  bg_color_class_by_intention,
  procedure_state_intention,
  stroke_color_class_by_intention,
} from "@lib/color";
import { cn } from "@lib/utils";
import { ProcedureDashboard } from "./dashboard";
import { Types } from "@monitor/client";

const useProcedure = (id?: string) =>
  useRead("ListProcedures", {}).data?.find((d) => d.id === id);

const ProcedureIcon = ({ id, size }: { id?: string; size: number }) => {
  const state = useProcedure(id)?.info.state;
  const color = stroke_color_class_by_intention(procedure_state_intention(state));
  return <Route className={cn(`w-${size} h-${size}`, state && color)} />;
};

export const ProcedureComponents: RequiredResourceComponents = {
  list_item: (id) => useProcedure(id),

  Dashboard: ProcedureDashboard,

  New: () => <NewResource type="Procedure" />,

  Table: ({ resources }) => (
    <ProcedureTable procedures={resources as Types.ProcedureListItem[]} />
  ),

  Icon: ({ id }) => <ProcedureIcon id={id} size={4} />,
  BigIcon: ({ id }) => <ProcedureIcon id={id} size={8} />,

  Status: {
    State: ({ id }) => {
      let state = useProcedure(id)?.info.state;
      const color = bg_color_class_by_intention(
        procedure_state_intention(state)
      );
      return (
        <Card className={cn("w-fit", color)}>
          <CardHeader className="py-0 px-2">{state}</CardHeader>
        </Card>
      );
    },
  },

  Info: {
    Stages: ({ id }) => <div>Stages: {useProcedure(id)?.info.stages}</div>,
  },

  Actions: {
    RunProcedure: ({ id }) => {
      const running = useRead(
        "GetProcedureActionState",
        { procedure: id },
        { refetchInterval: 5000 }
      ).data?.running;
      const { mutate, isPending } = useExecute("RunProcedure");
      const procedure = useProcedure(id);
      if (!procedure) return null;
      return (
        <ActionWithDialog
          name={procedure.name}
          title={running ? "Running" : "Run"}
          icon={<Route className="h-4 w-4" />}
          onClick={() => mutate({ procedure: id })}
          disabled={running || isPending}
          loading={running}
        />
      );
    },
  },

  Page: {},

  Config: ProcedureConfig,

  DangerZone: ({ id }) => <DeleteResource type="Procedure" id={id} />,
};
