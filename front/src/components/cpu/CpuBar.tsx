import "./Cpu.css";

import { CoreData } from "../../models/cpu";

type CoreBarProps = {
    coreData: CoreData;
};

export default function CoreBar(props: CoreBarProps) {
    const total =
        props.coreData.user +
        props.coreData.system +
        props.coreData.irq +
        props.coreData.softirq +
        props.coreData.idle +
        props.coreData.iowait;

    const idle = props.coreData.idle + props.coreData.iowait;
    const usage = ((total - idle) / total) * 100;

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
