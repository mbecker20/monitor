import { ConfigItem } from "@components/config/util";
import { Types } from "@monitor/client";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { keys } from "@lib/utils";

const format_mode = (m: string) => m.split("-").join(" ");

export const RestartModeSelector = ({
  selected,
  set,
}: {
  selected: Types.RestartMode | undefined;
  set: (input: Partial<Types.DeploymentConfig>) => void;
}) => (
  <ConfigItem label="Restart Mode">
    <Select
      value={selected || undefined}
      onValueChange={(restart: Types.RestartMode) => set({ restart })}
    >
      <SelectTrigger className="max-w-[150px] capitalize">
        <SelectValue placeholder="Select Type" />
      </SelectTrigger>
      <SelectContent>
        {keys(Types.RestartMode).map((mode) => (
          <SelectItem
            key={mode}
            value={Types.RestartMode[mode]}
            className="capitalize"
          >
            {format_mode(Types.RestartMode[mode])}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  </ConfigItem>
);