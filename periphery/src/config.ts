import { PeripherySecrets } from "@monitor/types";
import { getBooleanFromEnv, getNumberFromEnv, getStringFromEnv, readJSONFile } from "@monitor/util-node";
import { join } from "path";

export const SECRETS = readJSONFile("/secrets/secrets.json") as PeripherySecrets;
export const LOG = getBooleanFromEnv("LOG", true);
export const PORT = getNumberFromEnv("PORT", 8000);
// export const REGISTRY_URL = getStringFromEnv("REGISTRY_URL", "http://localhost:5000/");
export const SYSROOT = getStringFromEnv("SYSROOT", "/home/ubuntu/");
export const ROOT = "/monitor-root/";
export const CONTAINER_REPO_ROOT = join(ROOT, "repos");
export const SYS_REPO_ROOT = join(SYSROOT, "repos");

export const PM2_CLIENT_PORT = getNumberFromEnv("PM2_CLIENT_PORT", 4000);