import React from "react";
import clsx from "clsx";
import tooltipStyles from "./styles.module.css";

type FlagsState = "changed" | "unchanged" | boolean;
type ToolTipPosition = "top" | "bottom";
function Tooltip({
  children,
  text,
  toolTipPosition = "top",
}: {
  children: React.ReactNode;
    text: string;
    toolTipPosition?: ToolTipPosition;
  }) {
    
  const tooltipPositionClass =
    toolTipPosition === "top"
      ? tooltipStyles.tooltiptext
      : tooltipStyles.bottomTooltiptext;
  
  return (
    <div className={tooltipStyles.tooltip}>
      {children}
      <span className={tooltipPositionClass}>{text}</span>
    </div>
  );
}

const FlagsChanged: React.FC<{ state: FlagsState }> = ({ state }) => {
  if (state === "changed") {
    return (
      <span className="badge badge--primary badge--rounded h-20">
        <span className="badge__text">
          <Tooltip text="Changes">
            C
          </Tooltip>
        </span>
      </span>
    );
  } else if (state === "unchanged") {
    return (
      <span className="badge badge--secondary badge--rounded">
        <span className="badge__text">
          <Tooltip text="Doesn't change">
            NC
          </Tooltip>
        </span>
      </span>
    );
  } else if (state == true) {
    return (
      <span className="badge badge--success badge--rounded">
        <span className="badge__text">
          <Tooltip text="Changed to 1">
            1 
          </Tooltip>
        </span>
      </span>
    );
  } else if (state == false) {
    return (
      <span className="badge badge--danger badge--rounded">
        <span className="badge__text">
          <Tooltip text="Changed to 0">
            0
          </Tooltip>
        </span>
      </span>
    );
  } else {
    return (
      <span className="badge badge--warning badge--rounded">
        <span className="badge__text">Unknown</span>
      </span>
    );
  }
};

export default function FlagsChangedTable({
  carryFlag = null,
  zeroFlag = null,
  signFlag = null,
  overflowFlag = null,
  parityFlag = null,
  auxiliaryCarryFlag = null,
}: {
  carryFlag: FlagsState | null;
  zeroFlag: FlagsState | null;
  signFlag: FlagsState | null;
  overflowFlag: FlagsState | null;
  parityFlag: FlagsState | null;
  auxiliaryCarryFlag: FlagsState | null;
}) {
  const flags = {
    carryFlag,
    zeroFlag,
    signFlag,
    overflowFlag,
    parityFlag,
    auxiliaryCarryFlag,
  };

  return (
    // make the table like this
    // Flagname, flagname , flagname
    // 0, 0, 0
    <table className="table table--striped table--responsive">
      <thead>
        {/* only show if the value flagstate is null */}
        <tr>
          {Object.entries(flags).map(([flagName, flagState]) =>
            flagState == null ? null : (
              <th>
                <Tooltip text={flagName} toolTipPosition="bottom">
                  {flagName[0].toUpperCase()}
                </Tooltip>
              </th>
            )
          )}
        </tr>
      </thead>
      <tbody>
        <tr>
          {Object.values(flags).map((flagState) =>
            flagState == null ? null : (
              <td>
                <FlagsChanged state={flagState} />
              </td>
            )
          )}
        </tr>
      </tbody>
    </table>
  );
}
