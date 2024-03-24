import { CoreData } from "@/models/cpu";
import "./Cpu.css";
import CpuBar from "./CpuBar";

type CpuProps = {
    temp: number;
    cpuUsage: CoreData[];
};

export default function Cpu(props: CpuProps) {
    return (
        <div className="stats-container cpu-stats-container">
            <div className="temp">{props.temp}°C</div>
            <div className="bars-container w-100 align-items-center">
                {props.cpuUsage.length !== 0 && (
                    <div className="core-container">
                        <p className="cpu-bar-label">All</p>
                        <CpuBar coreData={props.cpuUsage[0]} />
                    </div>
                )}
                <div></div>
                {props.cpuUsage.slice(1).map((coreUsage, index) => (
                    <div className="core-container" key={index}>
                        <p className="cpu-bar-label">Core {index + 1}</p>
                        <CpuBar coreData={coreUsage} />
                    </div>
                ))}
            </div>
        </div>
    );
}
