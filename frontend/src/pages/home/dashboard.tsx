import { useRead } from "@lib/hooks";
import { Page, Section } from "@components/layouts";
import { AlertTriangle, Box, FolderTree } from "lucide-react";
import { DataTable } from "@ui/data-table";
import { Link } from "react-router-dom";
import { AlertLevel } from "@components/util";
import { Button } from "@ui/button";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { TagsSummary } from "@components/dashboard/tags";
import { ApiKeysSummary } from "@components/dashboard/api-keys";
import { atomWithStorage } from "jotai/utils";
import { useAtom } from "jotai";
import { fmt_date_with_minutes } from "@lib/formatting";
import { ResourceComponents } from "@components/resources";

export const Dashboard = () => {
  return (
    <Page title="">
      {/* <RecentlyViewed /> */}
      <OpenAlerts />
      <Resources />
    </Page>
  );
};

const openAtom = atomWithStorage("show-alerts-v0", true);

const OpenAlerts = () => {
  const [open, setOpen] = useAtom(openAtom);
  const alerts = useRead("ListAlerts", { query: { resolved: false } }).data
    ?.alerts;
  if (!alerts || alerts.length === 0) return null;
  return (
    <Section
      title="Open Alerts"
      icon={<AlertTriangle className="w-4 h-4" />}
      actions={
        <Button variant="ghost" onClick={() => setOpen(!open)}>
          {open ? "close" : "open"}
        </Button>
      }
    >
      {open && (
        <DataTable
          data={alerts ?? []}
          columns={[
            {
              header: "Target",
              cell: ({ row }) => {
                switch (row.original.target.type) {
                  case "Server":
                    return (
                      <Link to={`/servers/${row.original.target.id}`}>
                        <ResourceComponents.Server.Name
                          id={row.original.target.id}
                        />
                      </Link>
                    );
                  default:
                    return "Unknown";
                }
              },
            },
            {
              header: "Level",
              cell: ({ row }) => <AlertLevel level={row.original.level} />,
            },
            {
              header: "Alert",
              accessorKey: "variant",
            },
            {
              header: "Open Since",
              accessorFn: ({ ts }) => fmt_date_with_minutes(new Date(ts)),
            },
          ]}
        />
      )}
    </Section>
  );
};

const Resources = () => (
  <Section title="Resources" icon={<Box className="w-4 h-4" />} actions="">
    <div className="flex flex-col lg:flex-row gap-4 w-full">
      <div className="flex flex-col md:flex-row gap-4 w-full">
        <ResourceComponents.Server.Dashboard />
        <ResourceComponents.Deployment.Dashboard />
      </div>
      <div className="w-full lg:max-w-[50%]">
        <ResourceComponents.Build.Dashboard />
      </div>
    </div>
    <div className="flex flex-col lg:flex-row gap-4 w-full">
      <div className="w-full lg:max-w-[50%]">
        <Link to="/tree" className="w-full">
          <Card>
            <CardHeader className="justify-between">
              <div>
                <CardTitle>Tree</CardTitle>
                <CardDescription>
                  Visualize your servers / deployments
                </CardDescription>
              </div>
              <FolderTree className="w-4 h-4" />
            </CardHeader>
          </Card>
        </Link>
      </div>
      <div className="flex flex-col md:flex-row gap-4 w-full">
        <ResourceComponents.Repo.Dashboard />
        <ResourceComponents.Builder.Dashboard />
        {/* <TagsSummary />
        <ApiKeysSummary /> */}
      </div>
    </div>
    <div className="flex flex-col lg:flex-row gap-4 w-full">
      <div className="flex flex-col md:flex-row gap-4 w-full">
        <ResourceComponents.Alerter.Dashboard />
        <ResourceComponents.Procedure.Dashboard />
      </div>
      <div className="flex flex-col md:flex-row gap-4 w-full">
        <TagsSummary />
        <ApiKeysSummary />
      </div>
    </div>
  </Section>
);

// const RecentlyViewed = () => (
//   <Section
//     title="Recently Viewed"
//     icon={<History className="w-4 h-4" />}
//     actions=""
//   >
//     <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
//       {useRead("GetUser", {})
//         .data?.recently_viewed?.slice(0, 6)
//         .map(
//           (target) =>
//             target.type !== "System" && (
//               <ResourceCard target={target} key={target.id} />
//             )
//         )}
//     </div>
//   </Section>
// );