import {
  Accessor,
  Component,
  For,
  JSXElement,
  ParentComponent,
  Show,
} from "solid-js";
import { COLORS } from "../../style/colors";
import { SystemStats, SystemStatsRecord } from "../../types";
import {
  convertTsMsToLocalUnixTsInSecs,
  get_to_one_sec_divisor,
} from "../../util/helpers";
import { useLocalStorageToggle } from "../../util/hooks";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import LightweightChart, { LightweightValue } from "../shared/LightweightChart";
import s from "./stats.module.scss";

const CHART_HEIGHT = "250px";
const SMALL_CHART_HEIGHT = "150px";

const SingleStatChart: Component<{
  line?: LightweightValue[];
  header: string;
  headerRight?: JSXElement;
  label: string;
  color: string;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  return (
    <StatChartContainer
      header={p.header}
      headerRight={p.headerRight}
      small={p.small}
    >
      <Show when={p.line}>
        <LightweightChart
          class={s.LightweightChart}
          height={p.small ? SMALL_CHART_HEIGHT : CHART_HEIGHT}
          areas={[
            {
              line: p.line!,
              title: p.label,
              lineColor: p.color,
              topColor: `${p.color}B3`,
              bottomColor: `${p.color}0D`,
            },
          ]}
          disableScroll={p.disableScroll}
        />
      </Show>
    </StatChartContainer>
  );
};

const StatChartContainer: ParentComponent<{
  header: string;
  headerRight?: JSXElement;
  small?: boolean;
}> = (p) => {
  return (
    <Grid
      gap="0.5rem"
      class="card shadow full-width"
      style={{
        height: "fit-content",
        "padding-top": "0.5rem",
        "padding-bottom": "0.2rem",
      }}
    >
      <Flex justifyContent="space-between">
        <Show when={!p.small} fallback={<div>{p.header}</div>}>
          <h2>{p.header}</h2>
        </Show>
        {p.headerRight}
      </Flex>
      {p.children}
    </Grid>
  );
};

export const LoadChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  const line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(
          (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
        ),
        value: s.system_load || 0,
      };
    });
  };
  return (
    <SingleStatChart
      header="system load %"
      label="load %"
      color={COLORS.blue}
      line={line()}
      small={p.small}
      disableScroll={p.disableScroll}
    />
  );
};

export const CpuChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  const line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(
          (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
        ),
        value: s.cpu_perc,
      };
    });
  };
  return (
    <SingleStatChart
      header="cpu %"
      label="cpu %"
      color={COLORS.blue}
      line={line()}
      small={p.small}
      disableScroll={p.disableScroll}
    />
  );
};

export const CpuFreqChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  const line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(
          (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
        ),
        value: (s.cpu_freq_mhz || 0) / 1000,
      };
    });
  };
  return (
    <SingleStatChart
      header="cpu frequency"
      label="GHz"
      color={COLORS.blue}
      line={line()}
      small={p.small}
      disableScroll={p.disableScroll}
    />
  );
};

export const MemChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  const [absolute, toggleAbsolute] = useLocalStorageToggle("stats-mem-mode-v2");
  const symbol = () => (absolute() ? "GiB" : "%");
  const line = () => {
    if (absolute()) {
      return p.stats()?.map((s) => {
        return {
          time: convertTsMsToLocalUnixTsInSecs(
            (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
          ),
          value: s.mem_used_gb,
        };
      });
    } else {
      return p.stats()?.map((s) => {
        return {
          time: convertTsMsToLocalUnixTsInSecs(
            (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
          ),
          value: (100 * s.mem_used_gb) / s.mem_total_gb,
        };
      });
    }
  };
  return (
    <SingleStatChart
      header="memory"
      headerRight={
        <button
          class="green"
          style={{ padding: "0.2rem" }}
          onClick={toggleAbsolute}
        >
          {symbol()}
        </button>
      }
      label={`mem ${symbol()}`}
      color={COLORS.green}
      line={line()}
      small={p.small}
      disableScroll={p.disableScroll}
    />
  );
};

export const DiskChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  const [absolute, toggleAbsolute] =
    useLocalStorageToggle("stats-disk-mode-v2");
  const symbol = () => (absolute() ? "GiB" : "%");
  const line = () => {
    if (absolute()) {
      return p.stats()?.map((s) => {
        return {
          time: convertTsMsToLocalUnixTsInSecs(
            (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
          ),
          value: s.disk.used_gb,
        };
      });
    } else {
      return p.stats()?.map((s) => {
        return {
          time: convertTsMsToLocalUnixTsInSecs(
            (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
          ),
          value: (100 * s.disk.used_gb) / s.disk.total_gb,
        };
      });
    }
  };
  return (
    <SingleStatChart
      header="disk"
      headerRight={
        <button
          class="orange"
          style={{ padding: "0.2rem" }}
          onClick={toggleAbsolute}
        >
          {symbol()}
        </button>
      }
      label={`disk ${symbol()}`}
      color={COLORS.orange}
      line={line()}
      small={p.small}
      disableScroll={p.disableScroll}
    />
  );
};

export const NetworkRecvChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  const recv_line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(
          (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
        ),
        value:
          s.networks?.length || 0 > 0
            ? s.networks!.map((n) => n.recieved_kb).reduce((p, c) => p + c) /
              get_to_one_sec_divisor(s.polling_rate)!
            : 0,
      };
    });
  };
  return (
    <SingleStatChart
      header="network received kb/s"
      label="recv kb/s"
      color={COLORS.green}
      line={recv_line()}
      small={p.small}
      disableScroll={p.disableScroll}
    />
  );
};

export const NetworkSentChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  const sent_line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(
          (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
        ),
        value:
          s.networks?.length || 0 > 0
            ? s.networks!.map((n) => n.transmitted_kb).reduce((p, c) => p + c) /
              get_to_one_sec_divisor(s.polling_rate)!
            : 0,
      };
    });
  };
  return (
    <SingleStatChart
      header="network sent kb/s"
      label="sent kb/s"
      color={COLORS.red}
      line={sent_line()}
      small={p.small}
      disableScroll={p.disableScroll}
    />
  );
};

export const DiskReadChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
  gridFillers?: boolean;
}> = (p) => {
  const read_line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(
          (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
        ),
        value: s.disk.read_kb / get_to_one_sec_divisor(s.polling_rate)!,
      };
    });
  };
  return (
    <SingleStatChart
      header="disk read kb/s"
      label="read kb/s"
      color={COLORS.green}
      line={read_line()}
      small={p.small}
      disableScroll={p.disableScroll}
    />
  );
};

export const DiskWriteChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
  gridFillers?: boolean;
}> = (p) => {
  const write_line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(
          (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
        ),
        value: s.disk.write_kb / get_to_one_sec_divisor(s.polling_rate)!,
      };
    });
  };
  return (
    <SingleStatChart
      header="disk write kb/s"
      label="write kb/s"
      color={COLORS.red}
      line={write_line()}
      small={p.small}
      disableScroll={p.disableScroll}
    />
  );
};

export const TempuratureChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  const labels = () => {
    return p.stats()![p.stats()!.length - 1].components!.map((c) => c.label);
  };
  return (
    <For each={labels()}>
      {(component) => (
        <SingleTempuratureChart
          component={component}
          stats={p.stats}
          small={p.small}
          disableScroll={p.disableScroll}
        />
      )}
    </For>
  );
};

export const SingleTempuratureChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  component: string;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  const line = () => {
    return p.stats()?.map((s) => {
      const temp = s.components!.find((c) => c.label === p.component)?.temp;
      return {
        time: convertTsMsToLocalUnixTsInSecs(
          (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
        ),
        value: temp || 0,
      };
    });
  };
  return (
    <SingleStatChart
      header={p.component}
      label="temp"
      color={COLORS.red}
      line={line()}
      small={p.small}
      disableScroll={p.disableScroll}
    />
  );
};
