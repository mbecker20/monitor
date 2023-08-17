import { DoubleInput } from "@components/config/util";
import { Types } from "@monitor/client";

export const VolumesConfig = ({
  volumes,
  set,
}: {
  volumes: Types.Conversion[];
  set: (input: Partial<Types.DeploymentConfig>) => void;
}) => (
  <DoubleInput
    values={volumes}
    leftval="local"
    leftpl="Local"
    rightval="container"
    rightpl="Container"
    addName="Volume"
    onLeftChange={(local, i) => {
      volumes[i].local = local;
      set({ volumes: [...volumes] });
    }}
    onRightChange={(container, i) => {
      volumes[i].container = container;
      set({ volumes: [...volumes] });
    }}
    onAdd={() =>
      set({ volumes: [...(volumes ?? []), { container: "", local: "" }] })
    }
    onRemove={(idx) =>
      set({ volumes: [...volumes.filter((_, i) => i !== idx)] })
    }
  />
);