import { formatBytes } from "@/lib/bytes";
import { CoreData } from "@/models/cpu";
import { ProcessData } from "@/models/proc";
import { ChangeEvent, useState } from "react";
import "./Procs.css";

enum ProcessProperty {
    PID = 0,
    Name = 1,
    Threads = 2,
    Memory = 3,
    CPU = 4,
}

type ProcessOrdering = {
    ord: ProcessProperty;
    rev: boolean;
};

type ProcessOrderingFunction = (p1: ProcessData, p2: ProcessData) => number;
const orderingFromProperty = (
    processProperty: ProcessProperty,
): ProcessOrderingFunction => {
    switch (processProperty) {
        case ProcessProperty.PID:
            return (p1, p2) => p2.pid - p1.pid;
        case ProcessProperty.Name:
            return (p1, p2) => p1.name.localeCompare(p2.name);
        case ProcessProperty.Threads:
            return (p1, p2) => p2.threads - p1.threads;
        case ProcessProperty.Memory:
            return (p1, p2) => p2.mem - p1.mem;
        case ProcessProperty.CPU:
            return (p1, p2) => p2.cpu_usage - p1.cpu_usage;
        default:
            console.log(`Unknown ordering value passed: ${processProperty}`);
            return (_p1, _p2) => 0;
    }
};

type ProcProps = {
    procs: ProcessData[];
    mainCpuUsage: CoreData;
};

const pidLabelDefault = "PID";
const nameLabelDefault = "Name";
const threadsLabelDefault = "Thds";
const memLabelDefault = "Mem";
const cpuLabelDefault = "CPU";

export default function Procs(props: ProcProps) {
    const [ordering, setOrdering] = useState<ProcessOrdering>({
        ord: ProcessProperty.Memory,
        rev: false,
    });
    const [search, setSearch] = useState("");

    const total =
        props.mainCpuUsage.user +
        props.mainCpuUsage.nice +
        props.mainCpuUsage.system +
        props.mainCpuUsage.idle +
        props.mainCpuUsage.iowait +
        props.mainCpuUsage.irq +
        props.mainCpuUsage.softirq +
        props.mainCpuUsage.steal +
        props.mainCpuUsage.guest +
        props.mainCpuUsage.guest_nice;

    const visibleProcs = props.procs
        .filter(
            (p) =>
                p.name.toLowerCase().includes(search.toLowerCase()) ||
                p.pid.toString().startsWith(search),
        )
        .sort(orderingFromProperty(ordering.ord));

    if (ordering.rev) {
        visibleProcs.reverse();
    }

    const sortProcessesBy = (prop: ProcessProperty) => {
        setOrdering({
            ord: prop,
            rev: ordering.ord === prop ? !ordering.rev : false,
        });
    };

    const propertyLabel = (
        prop: ProcessProperty,
        currentOrdering: ProcessOrdering,
        defaultPropertyLabel: string,
        visibleProcs: ProcessData[],
    ): string => {
        if (visibleProcs.length === 0) {
            return "";
        }

        if (ordering.ord !== prop) {
            return defaultPropertyLabel;
        }

        if (prop === ProcessProperty.Name) {
            return `${defaultPropertyLabel}${currentOrdering.rev ? "▲" : "▼"}`;
        } else {
            return `${currentOrdering.rev ? "▲" : "▼"}${defaultPropertyLabel}`;
        }
    };

    const handleSearchChange = (event: ChangeEvent<HTMLInputElement>) => {
        setSearch(event.target.value);
    };

    return (
        <div className="stats-container flex-column">
            <div
                id="proc-header-container"
                className="d-flex justify-content-between"
            >
                <p>{props.procs.length} active processes</p>
                <div className="search-bar-container">
                    <input
                        type="text"
                        id="search-bar"
                        placeholder="Search..."
                        className="form-control form-control-sm"
                        onChange={handleSearchChange}
                    />
                </div>
            </div>
            <div className="proc-container d-flex flex-row">
                <div className="pid-col proc-col d-flex flex-column">
                    <div
                        className="text-nowrap"
                        onClick={() => sortProcessesBy(ProcessProperty.PID)}
                    >
                        <span
                            className={`col-content${
                                ordering.ord === ProcessProperty.PID
                                    ? " text-decoration-underline"
                                    : ""
                            }`}
                        >
                            {propertyLabel(
                                ProcessProperty.PID,
                                ordering,
                                pidLabelDefault,
                                visibleProcs,
                            )}
                        </span>
                    </div>
                    <div></div>
                    {visibleProcs.map((p, i) => (
                        <div className="col-content" key={i}>
                            {p.pid}
                        </div>
                    ))}
                </div>
                <div className="name-col proc-col d-flex flex-column">
                    <div
                        className="text-nowrap"
                        onClick={() => sortProcessesBy(ProcessProperty.Name)}
                    >
                        <span
                            className={`col-content${
                                ordering.ord === ProcessProperty.Name
                                    ? " text-decoration-underline"
                                    : ""
                            }`}
                        >
                            {propertyLabel(
                                ProcessProperty.Name,
                                ordering,
                                nameLabelDefault,
                                visibleProcs,
                            )}
                        </span>
                    </div>
                    <div></div>
                    {visibleProcs.map((p, i) => (
                        <p className="col-content" key={i}>
                            {p.name}
                        </p>
                    ))}
                </div>
                <div className="threads-col proc-col d-flex flex-column">
                    <div
                        className="text-nowrap threads-label"
                        onClick={() => sortProcessesBy(ProcessProperty.Threads)}
                    >
                        <span
                            className={`col-content${
                                ordering.ord === ProcessProperty.Threads
                                    ? " text-decoration-underline"
                                    : ""
                            }`}
                        >
                            {propertyLabel(
                                ProcessProperty.Threads,
                                ordering,
                                threadsLabelDefault,
                                visibleProcs,
                            )}
                        </span>
                    </div>
                    <div></div>
                    {visibleProcs.map((p, i) => (
                        <p className="col-content" key={i}>
                            {p.threads}
                        </p>
                    ))}
                </div>
                <div className="memory-col proc-col d-flex flex-column">
                    <div
                        className="text-nowrap mem-label"
                        onClick={() => sortProcessesBy(ProcessProperty.Memory)}
                    >
                        <span
                            className={`col-content${
                                ordering.ord === ProcessProperty.Memory
                                    ? " text-decoration-underline"
                                    : ""
                            }`}
                        >
                            {propertyLabel(
                                ProcessProperty.Memory,
                                ordering,
                                memLabelDefault,
                                visibleProcs,
                            )}
                        </span>
                    </div>
                    <div></div>
                    {visibleProcs.map((p, i) => (
                        <p className="col-content" key={i}>
                            {formatBytes(p.mem, {
                                short: true,
                                space: false,
                                roundTreshold: 10,
                                absoluteRoundTreshold: 1024 ** 3,
                                roundingDigits: 1,
                            })}
                        </p>
                    ))}
                </div>
                <div className="cpu-col proc-col d-flex flex-column">
                    <div
                        className="text-nowrap cpu-label"
                        onClick={() => sortProcessesBy(ProcessProperty.CPU)}
                    >
                        <span
                            className={`col-content${
                                ordering.ord === ProcessProperty.CPU
                                    ? " text-decoration-underline"
                                    : ""
                            }`}
                        >
                            {propertyLabel(
                                ProcessProperty.CPU,
                                ordering,
                                cpuLabelDefault,
                                visibleProcs,
                            )}
                        </span>
                    </div>
                    <div></div>
                    {visibleProcs.map((p, i) => (
                        <p className="col-content cpu-percs" key={i}>
                            {((p.cpu_usage / total) * 100)
                                .toFixed(1)
                                .padStart(5, " ")}
                            %
                        </p>
                    ))}
                </div>
            </div>
        </div>
    );
}
