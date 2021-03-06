import { exec } from "child_process";
import { promisify } from "util";
import { CommandLogError } from "@monitor/types";
import { prettyStringify } from "@monitor/util";

export const pExec = promisify(exec);

export async function execute(
  command: string,
  commandForLog?: string
): Promise<CommandLogError> {
  try {
    return {
      command: commandForLog || command,
      log: await pExec(command),
      isError: false,
    };
  } catch (err) {
    return {
      command: commandForLog || command,
      log: { stderr: prettyStringify(err) },
      isError: true,
    };
  }
}
