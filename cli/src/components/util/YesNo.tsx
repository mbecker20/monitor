import React, { ReactNode } from "react";
import LabelledSelector from "./LabelledSelector";

const YesNo = ({
  label,
  onYes,
  onNo,
  onSelect,
  vertical,
  labelColor,
	noYes,
  onEsc
}: {
  label: ReactNode;
  onYes?: () => void;
  onNo?: () => void;
  onSelect?: (res: "yes" | "no") => void;
  vertical?: boolean;
  labelColor?: "green" | "white";
  noYes?: boolean;
  onEsc?: () => void;
}) => {
  return (
    <LabelledSelector
      label={label}
      items={noYes ? ["no", "yes"] : ["yes", "no"]}
      onSelect={(item) => {
        if (item === "yes") {
          onYes && onYes();
        } else {
          onNo && onNo();
        }
        onSelect && onSelect(item as "yes" | "no");
      }}
      vertical={vertical}
      labelColor={labelColor}
      onEsc={onEsc}
    />
  );
};

export default YesNo;
