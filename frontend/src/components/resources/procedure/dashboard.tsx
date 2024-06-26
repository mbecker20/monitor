import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { PieChart } from "react-minimal-pie-chart";
import { useRead } from "@lib/hooks";
import { Route } from "lucide-react";
import { Link } from "react-router-dom";
import { cn } from "@lib/utils";
import {
  hex_color_by_intention,
  text_color_class_by_intention,
} from "@lib/color";

export const ProcedureDashboard = () => {
  const summary = useRead("GetProceduresSummary", {}).data;

  return (
    <Link to="/procedures">
      <Card className="hover:bg-accent/50 transition-colors cursor-pointer w-[300px]">
        <CardHeader>
          <div className="flex justify-between">
            <div>
              <CardTitle>Procedures</CardTitle>
              <CardDescription>{summary?.total} Total</CardDescription>
            </div>
            <Route className="w-4 h-4" />
          </div>
        </CardHeader>
        <CardContent className="flex h-[200px] items-center justify-between gap-4">
          <div className="flex flex-col gap-2 text-muted-foreground w-full text-nowrap">
            <CardDescription className="flex items-center gap-2">
              <span
                className={cn(
                  text_color_class_by_intention("Good"),
                  "font-bold"
                )}
              >
                {summary?.ok}{" "}
              </span>
              Ok
            </CardDescription>
            <CardDescription className="flex items-center gap-2">
              <span
                className={cn(
                  text_color_class_by_intention("Warning"),
                  "font-bold"
                )}
              >
                {summary?.running}{" "}
              </span>
              Running
            </CardDescription>
            <CardDescription className="flex items-center gap-2">
              <span
                className={cn(
                  text_color_class_by_intention("Critical"),
                  "font-bold"
                )}
              >
                {summary?.failed}{" "}
              </span>
              Failed
            </CardDescription>
            <CardDescription className="flex items-center gap-2">
              <span
                className={cn(
                  text_color_class_by_intention("Unknown"),
                  "font-bold"
                )}
              >
                {summary?.unknown}{" "}
              </span>
              Unknown
            </CardDescription>
          </div>
          <div className="flex justify-end items-center w-full">
            <PieChart
              className="w-32 h-32"
              radius={42}
              lineWidth={30}
              data={[
                {
                  color: hex_color_by_intention("Good"),
                  value: summary?.ok ?? 0,
                  title: "ok",
                  key: "ok",
                },
                {
                  color: hex_color_by_intention("Warning"),
                  value: summary?.running ?? 0,
                  title: "running",
                  key: "running",
                },
                {
                  color: hex_color_by_intention("Critical"),
                  value: summary?.failed ?? 0,
                  title: "failed",
                  key: "failed",
                },
                {
                  color: hex_color_by_intention("Unknown"),
                  value: summary?.unknown ?? 0,
                  title: "unknown",
                  key: "unknown",
                },
              ]}
            />
          </div>
        </CardContent>
      </Card>
    </Link>
  );
};
