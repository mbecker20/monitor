import { ALERT } from "@monitor/util";
import { WebSocket } from "ws";

export function toDashedName(name: string) {
  return name.toLowerCase().replaceAll(" ", "-");
}

export function sendAlert(
  client: WebSocket,
  status: "good" | "bad" | "ok",
  message: string
) {
  client.send(JSON.stringify({ type: ALERT, status, message }))
}