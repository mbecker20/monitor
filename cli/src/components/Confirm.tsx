import React, { Fragment } from "react";
import { useConfig, useMainSequence } from "../cli";
import EnterToContinue from "./util/EnterToContinue";
import { useEsc } from "../util/hooks";
import { Box, Newline, Text } from "ink";
import View from "./deployment-config/View";
import ViewCoreOrPeriphery from "./core-or-periphery/ViewCoreOrPeriphery";

const Confirm = () => {
  const { config } = useConfig();
  const { next, prev } = useMainSequence();
  useEsc(prev);
  return (
    <Box flexDirection="column">
      {config.mongo && (
        <Fragment>
          <Text color="cyan" bold>
            mongo:
          </Text>
          <View url={config.mongo.url} config={config.mongo.startConfig} />
          <Newline />
        </Fragment>
      )}

      {/* {config.registry && (
        <Fragment>
          <Text color="cyan" bold>
            registry:
          </Text>
          <View
            url={config.registry.url!}
            config={config.registry.startConfig}
          />
          <Newline />
        </Fragment>
      )} */}

      {config.core && (
        <Fragment>
          <Text color="cyan" bold>
            monitor core:
          </Text>
          <ViewCoreOrPeriphery config={config.core} />
          <Newline />
        </Fragment>
      )}

      {config.periphery && (
        <Fragment>
          <Text color="cyan" bold>
            monitor periphery:
          </Text>
          <ViewCoreOrPeriphery config={config.periphery} />
          <Newline />
        </Fragment>
      )}

      <EnterToContinue pressEnterTo="begin setup" onEnter={next} />
    </Box>
  );
};

export default Confirm;
