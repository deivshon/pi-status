import "./Cpu.css";

import { useState } from "react";
import { useEffect } from "react";
import { CoreData } from "./models";

type CoreBarProps = {
    coreData: CoreData;
};

export default function CoreBar(props: CoreBarProps) {
    const [usage, setUsage] = useState(0);

    useEffect(() => {
        let total =
            props.coreData.user +
            props.coreData.system +
            props.coreData.irq +
            props.coreData.softirq +
            props.coreData.idle +
            props.coreData.iowait;

        let idle = props.coreData.idle + props.coreData.iowait;

        setUsage(((total - idle) / total) * 100);
    });

    return (
        <div className="progress w-100">
            <div
                className="progress-bar cpu-bar"
                role="progressbar"
                style={{ width: `${usage}%` }}
            ></div>
        </div>
    );
}
