import { useState } from "react";
import { useWrite } from "@hooks";
import { Input } from "@ui/input";
import { NewResource } from "@components/new-resource";
import { useNavigate } from "react-router-dom";

export const NewBuilder = ({
  open,
  set,
}: {
  open: boolean;
  set: (b: false) => void;
}) => {
  const nav = useNavigate();
  const { mutate, isLoading } = useWrite("CreateBuilder", {
    onSuccess: (d) => {
      set(false);
      nav(`/builders/${d._id?.$oid}`);
    },
  });

  const [name, setName] = useState("");

  return (
    <NewResource
      type="Builder"
      open={open}
      loading={isLoading}
      set={set}
      onSuccess={() => mutate({ name, config: { type: "Aws", params: {} } })}
    >
      <div className="flex items-center justify-between">
        <div>Builder Name</div>
        <Input
          className="max-w-[50%]"
          placeholder="Builder Name"
          name={name}
          onChange={(e) => setName(e.target.value)}
        />
      </div>
    </NewResource>
  );
};