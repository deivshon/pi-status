import { formatBytes } from "@/lib/bytes";
import { twColors } from "@/lib/consts";
import { ProgressBar } from "../shared/progress-bar";

type RamProps = {
    total: number;
    value: number;
    label: string;
};

export default function RamBar(props: RamProps) {
    const statPerc = props.total === 0 ? 0 : (props.value / props.total) * 100;

    return (
        <div className="flex w-full max-w-[1000px] flex-col gap-1">
            <div className="flex justify-between text-[0.9rem]">
                <p>{props.label}</p>
                <p>
                    {props.value !== 0
                        ? `${formatBytes(props.value)} (${statPerc.toFixed(
                              2,
                          )}%)`
                        : "0 (0%)"}
                </p>
            </div>
            <ProgressBar
                percentage={statPerc}
                className="h-4"
                bgColor={twColors["progress"]}
                fgColor={twColors["ayu-green"]}
            />
        </div>
    );
}
