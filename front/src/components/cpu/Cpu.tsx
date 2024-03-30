import { CoreData } from "@/models/cpu";
import CpuBar from "./CpuBar";

type CpuProps = {
    temp: number;
    cpuUsage: CoreData[];
};

export default function Cpu(props: CpuProps) {
    return (
        <div className="flex flex-col gap-3">
            <div className="temp">{props.temp}°C</div>
            <div className="flex w-full flex-col items-center gap-[0.75rem]">
                {props.cpuUsage.length !== 0 && (
                    <div className="mb-2 flex w-full max-w-[1000px] items-center gap-2 whitespace-pre px-1">
                        <p>All</p>
                        <CpuBar coreData={props.cpuUsage[0]} />
                    </div>
                )}
                {props.cpuUsage.slice(1).map((coreUsage, index) => (
                    <div
                        className="flex w-full max-w-[1000px] items-center gap-2 whitespace-pre px-1"
                        key={index}
                    >
                        <p>Core {index + 1}</p>
                        <CpuBar coreData={coreUsage} />
                    </div>
                ))}
            </div>
        </div>
    );
}
