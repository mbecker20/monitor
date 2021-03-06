import React, { Fragment } from "react";
import { Box, Newline, Text } from "ink";
import { join, resolve } from "path";
import { useConfig, useMainSequence } from "../../cli";
import { useEsc, useStore } from "../../util/hooks";
import {
  DEFAULT_PERIPHERY_PORT,
  DEFAULT_PORT,
  RESTART_MODES,
} from "../../config";
import EnterToContinue from "../util/EnterToContinue";
import { ControlledInput } from "../util/Input";
import NumberInput from "../util/NumberInput";
import { CoreOrPeripheryConfig } from "../../types";
import LabelledSelector from "../util/LabelledSelector";
import {
  noTrailingSlash,
  toDashedName,
  trailingSlash,
} from "../../util/helpers/general";

type Stage =
  | "name"
  | "secret"
  | "sysroot"
  | "host"
  | "port"
  | "restart"
  | "confirm";

const CoreOrPeriphery = ({ type }: { type: "core" | "periphery" }) => {
  const { set } = useConfig();
  const { next, prev } = useMainSequence();
  const isCore = type === "core";
  const [config, setConfig, setMany] = useStore<
    Partial<CoreOrPeripheryConfig> & { stage: Stage }
  >({
    stage: "name",
    name: isCore ? "monitor-core" : "monitor-periphery",
  });
  const { stage, name, secretVolume, host, port, restart, sysroot } = config;
  useEsc(() => {
    switch (stage) {
      case "name":
        prev();
        break;

      case "secret":
        setConfig("stage", "name");
        break;

      case "sysroot":
        setConfig("stage", "secret");
        break;

      case "host":
        setConfig("stage", "sysroot");
        break;

      case "port":
        setMany(["stage", isCore ? "host" : "sysroot"]);
        break;

      case "restart":
        setMany(["stage", "port"]);
        break;

      case "confirm":
        setMany(["stage", "restart"], ["restart", undefined]);
        break;
    }
  });
  return (
    <Box flexDirection="column">
      <Text color="green">
        name:{" "}
        <Text color="white">
          {stage === "name" ? (
            <ControlledInput
              value={name!}
              onChange={(name) => setConfig("name", name)}
              onSubmit={(name) => {
                setMany(["stage", "secret"], ["name", name]);
              }}
            />
          ) : (
            name
          )}
        </Text>
      </Text>

      {stage === "secret" && (
        <Text color="green">
          secrets folder:{" "}
          <Text color="white">
            <ControlledInput
              value={secretVolume || join(resolve("."), "/secrets")}
              onChange={(volume) => setConfig("secretVolume", volume)}
              onSubmit={(volume) => {
                setMany(["stage", "sysroot"], ["secretVolume", volume]);
              }}
            />
          </Text>
        </Text>
      )}

      {(secretVolume || undefined) && stage !== "secret" && (
        <Text color="green">
          secrets folder: <Text color="white">{secretVolume}</Text>
        </Text>
      )}

      {stage === "sysroot" && (
        <Text color="green">
          system root folder:{" "}
          <Text color="white">
            <ControlledInput
              value={sysroot || join(resolve("."), "/monitor")}
              onChange={(sysroot) => setConfig("sysroot", sysroot)}
              onSubmit={(sysroot) => {
                setMany(["stage", isCore ? "host" : "port"], ["sysroot", trailingSlash(sysroot)]);
              }}
            />
          </Text>
        </Text>
      )}

      {sysroot && stage !== "sysroot" && (
        <Text color="green">
          system root: <Text color="white">{sysroot}</Text>
        </Text>
      )}

      {stage === "host" && (
        <Text color="green">
          host address:{" "}
          <Text color="white">
            <ControlledInput
              value={host || "http://localhost:9000"}
              onChange={(host) => setConfig("host", host)}
              onSubmit={(host) => {
                setMany(["stage", "port"], ["host", noTrailingSlash(host)]);
              }}
            />
          </Text>
        </Text>
      )}

      {host && stage !== "host" && (
        <Text color="green">
          host address: <Text color="white">{host}</Text>
        </Text>
      )}

      {stage === "port" && (
        <Text color="green">
          port:{" "}
          <Text color="white">
            <NumberInput
              initialValue={
                port ||
                (type === "core" ? DEFAULT_PORT : DEFAULT_PERIPHERY_PORT)
              }
              onSubmit={(port) => {
                setMany(["stage", "restart"], ["port", port]);
              }}
            />
          </Text>
        </Text>
      )}

      {port && stage !== "port" && (
        <Text color="green">
          port: <Text color="white">{port}</Text>
        </Text>
      )}

      {stage === "restart" && (
        <LabelledSelector
          label="restart: "
          items={RESTART_MODES}
          onSelect={(restart) => {
            setMany(
              ["stage", "confirm"],
              [
                "restart",
                restart === "don't restart" ? "no" : toDashedName(restart),
              ]
            );
          }}
        />
      )}

      {restart && (
        <Text color="green">
          restart: <Text color="white">{restart}</Text>
        </Text>
      )}

      {stage === "confirm" && (
        <Fragment>
          <Newline />
          <EnterToContinue
            onEnter={() => {
              set(type, {
                name: name!,
                secretVolume: secretVolume!,
                port: Number(port),
                restart: restart!,
                sysroot: sysroot!,
                host,
              });
              next();
            }}
          />
        </Fragment>
      )}
    </Box>
  );
};

export default CoreOrPeriphery;
