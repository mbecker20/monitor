import { ConfirmButton } from "@components/util";
import { useExecute, useRead } from "@lib/hooks";
import { useToast } from "@ui/use-toast";
import { Ban, Hammer, Loader2 } from "lucide-react";

export const RunBuild = ({id}: {id: string}) => {
	const { toast } = useToast();
  const building = useRead("GetBuildActionState", { build: id }).data?.building;
  const { mutate: run_mutate, isPending: runPending } = useExecute("RunBuild", {
    onMutate: () => {
      toast({ title: "Run Build Sent" });
    },
  });
  const { mutate: cancel_mutate, isPending: cancelPending } = useExecute(
    "CancelBuild",
    {
      onMutate: () => {
        toast({ title: "Cancel Build Sent" });
      },
      onSuccess: () => {
        toast({ title: "Build Cancelled" });
      },
    }
  );
  if (building) {
    return (
      <ConfirmButton
        title="Cancel Build"
        variant="destructive"
        icon={<Ban className="h-4 w-4" />}
        onClick={() => cancel_mutate({ build: id })}
        disabled={cancelPending}
      />
    );
  } else {
    return (
      <ConfirmButton
        title="Build"
        icon={
          runPending ? (
            <Loader2 className="w-4 h-4 animate-spin" />
          ) : (
            <Hammer className="h-4 w-4" />
          )
        }
        onClick={() => run_mutate({ build: id })}
        disabled={runPending}
      />
    );
  }
}