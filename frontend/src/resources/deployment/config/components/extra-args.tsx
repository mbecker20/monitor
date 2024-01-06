import { ConfigItem } from "@components/config/util";
import { DeploymentConfig } from "@monitor/client/dist/types";
import { Button } from "@ui/button";
import { Input } from "@ui/input";
import { MinusCircle, PlusCircle } from "lucide-react";

export const ExtraArgs = ({
  args,
  set,
}: {
  args: string[];
  set: (update: Partial<DeploymentConfig>) => void;
}) => {
  return (
    <ConfigItem label="Extra Args" className="items-start">
      <div className="flex flex-col gap-4 w-full max-w-[400px]">
        {args.map((arg, i) => (
          <div className="w-full flex gap-4" key={i}>
            <Input
              value={arg}
              placeholder="--extra-arg=value"
              onChange={(e) => {
                args[i] = e.target.value;
                set({ extra_args: [...args] });
              }}
            />
            <Button
              variant="outline"
              intent="warning"
              onClick={() =>
                set({ extra_args: [...args.filter((_, idx) => idx !== i)] })
              }
            >
              <MinusCircle className="w-4 h-4" />
            </Button>
          </div>
        ))}

        <Button
          variant="outline"
          intent="success"
          className="flex items-center gap-2 w-[200px] place-self-end"
          onClick={() => set({ extra_args: [...args, ""] })}
        >
          <PlusCircle className="w-4 h-4" /> Add Extra Arg
        </Button>
      </div>
    </ConfigItem>
  );
};