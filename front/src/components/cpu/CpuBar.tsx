import { twColors } from "@/lib/consts";
import { CoreData } from "@/models/cpu";
import { ProgressBar } from "../shared/progress-bar";

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
        <ProgressBar
            percentage={usage}
            className="h-3.5"
            bgColor={twColors["progress"]}
            fgColor={twColors["ayu-purple"]}
        />
    );
}
