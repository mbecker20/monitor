import { ConfigItem } from "mogh_ui";
import { Group, TextInput } from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { useEffect, useState } from "react";

export function MinUpdateAge({
  arg,
  set,
  disabled,
  description,
}: {
  arg: number;
  set: (input: { min_update_age_hours: number }) => void;
  disabled: boolean;
  description?: string;
}) {
  const [input, setInput] = useState(arg.toString());
  useEffect(() => {
    setInput(arg.toString());
  }, [arg]);
  const num = Number(input);
  const error =
    num >= 0 ? undefined : "Minimum update age must be a positive number";
  return (
    <ConfigItem
      label="Minimum Update Age"
      description={
        description ??
        "Require the image to be at least this old before Auto Update deploys it. 0 disables the check."
      }
    >
      <Group gap="xs">
        <TextInput
          w={100}
          placeholder="0"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onBlur={(e) => {
            const num = Number(e.target.value);
            if (num >= 0) {
              set({ min_update_age_hours: num });
            } else {
              notifications.show({
                message: "Minimum update age must be a positive number",
                color: "red",
              });
            }
          }}
          error={error}
          disabled={disabled}
        />
        hours
      </Group>
    </ConfigItem>
  );
}
