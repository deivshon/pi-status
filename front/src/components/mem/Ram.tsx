import "./Ram.css";

import { formatBytes } from "../../utils";

import { useState, useEffect } from "react";

type RamProps = {
    total: number;
    value: number;
    label: string;
};

export default function Ram(props: RamProps) {
    const [statPerc, setStatPerc] = useState(0);

    useEffect(() => {
        if (props.total != 0) setStatPerc((props.value / props.total) * 100);
    }, [props.value, props.total]);

    return (
        <div className="d-flex ram-stats-container flex-column w-100">
            <div className="d-flex justify-content-between ram-text">
                <div>{props.label}</div>
                <div>
                    {props.value != 0
                        ? `${formatBytes(props.value, {})} (${statPerc.toFixed(
                              2,
                          )}%)`
                        : "0 (0%)"}
                </div>
            </div>
            <div className="progress w-100">
                <div
                    className="progress-bar ram-bar"
                    role="progressbar"
                    style={{ width: `${statPerc}%` }}
                ></div>
            </div>
        </div>
    );
}
