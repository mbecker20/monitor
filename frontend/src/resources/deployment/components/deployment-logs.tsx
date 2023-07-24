import { Button } from "@ui/button";
import { Card, CardHeader, CardContent } from "@ui/card";
import { Tabs, TabsList, TabsTrigger } from "@ui/tabs";
// import { useDeploymentLog } from "@hooks/deployments";
import { TabsContent } from "@radix-ui/react-tabs";
import { AlertOctagon, ChevronDown } from "lucide-react";
import { useEffect } from "react";
import { useRead } from "@hooks";

const scroll_to_bottom = (id: string) => () =>
  document
    .getElementById(id)
    ?.scrollIntoView({ behavior: "smooth", block: "end", inline: "nearest" });

export const DeploymentLogs = ({ deploymentId }: { deploymentId: string }) => {
  const { data, refetch } = useRead({
    type: "GetLog",
    params: { deployment_id: deploymentId, tail: 200 },
  });

  useEffect(() => {
    const handle = setInterval(() => refetch(), 30000);
    return () => clearInterval(handle);
  }, [refetch]);

  useEffect(() => {
    scroll_to_bottom("stdout")();
    scroll_to_bottom("stderr")();
  }, [data]);

  return (
    <Tabs defaultValue="stdout">
      <Card>
        <CardHeader className="flex-row justify-end">
          <TabsList>
            <TabsTrigger value="stdout" onClick={scroll_to_bottom("stdout")}>
              Out
            </TabsTrigger>
            <TabsTrigger value="stderr" onClick={scroll_to_bottom("stderr")}>
              Err
              {data?.stderr && (
                <AlertOctagon className="w-4 h-4 ml-2 stroke-red-500" />
              )}
            </TabsTrigger>
          </TabsList>
        </CardHeader>
        {["stdout", "stderr"].map((t) => (
          <TabsContent key={t} className="h-full relative" value={t}>
            <CardContent className="h-[50vh] overflow-y-scroll">
              <pre id={t}>
                {data?.[t as keyof typeof data] || `no ${t} logs`}
              </pre>
            </CardContent>
            <Button
              className="absolute bottom-8 right-12"
              onClick={scroll_to_bottom(t)}
            >
              <ChevronDown className="h-4 w-4" />
            </Button>
          </TabsContent>
        ))}
      </Card>
    </Tabs>
  );
};