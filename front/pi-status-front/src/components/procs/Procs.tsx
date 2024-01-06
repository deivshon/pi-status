import { useState, useEffect, useMemo } from "react";

import { formatBytes } from "../../utils";
import { CoreData } from "../cpu/models";
import "./Procs.css";

enum ProcessProperty {
    PID = 0,
    NAME = 1,
    THREADS = 2,
    MEM = 3,
    CPU = 4,
}

type ProcessOrdering = {
    ord: ProcessProperty;
    rev: boolean;
};

type ProcessOrderingFunction = (p1: ProcessData, p2: ProcessData) => number;
const orderingFromProperty = (
    processProperty: ProcessProperty
): ProcessOrderingFunction => {
    switch (processProperty) {
        case ProcessProperty.PID:
            return (p1, p2) => p2.pid - p1.pid;
        case ProcessProperty.NAME:
            return (p1, p2) => p1.name.localeCompare(p2.name);
        case ProcessProperty.THREADS:
            return (p1, p2) => p2.threads - p1.threads;
        case ProcessProperty.MEM:
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
    const [currentTotal, setTotal] = useState(1);
    const [ordering, setOrdering] = useState<ProcessOrdering>({
        ord: ProcessProperty.MEM,
        rev: false,
    });

    useEffect(() => {
        setTotal(
            props.mainCpuUsage.user +
                props.mainCpuUsage.nice +
                props.mainCpuUsage.system +
                props.mainCpuUsage.idle +
                props.mainCpuUsage.iowait +
                props.mainCpuUsage.irq +
                props.mainCpuUsage.softirq +
                props.mainCpuUsage.steal +
                props.mainCpuUsage.guest +
                props.mainCpuUsage.guest_nice
        );
    }, [props.mainCpuUsage]);

    useMemo(() => {
        const sortProcs = () => {
            if (ordering.rev)
                props.procs = props.procs
                    .sort(orderingFromProperty(ordering.ord))
                    .reverse();
            else
                props.procs = props.procs.sort(
                    orderingFromProperty(ordering.ord)
                );
        };

        sortProcs();
    }, [props.procs, ordering]);

    const sortProcessesBy = (prop: ProcessProperty) => {
        setOrdering({
            ord: prop,
            rev: ordering.ord === prop ? !ordering.rev : false,
        });
    };

    const propertyLabel = (
        prop: ProcessProperty,
        currentOrdering: ProcessOrdering,
        defaultPropertyLabel: string
    ): string => {
        if (ordering.ord !== prop) {
            return defaultPropertyLabel;
        }

        if (prop === ProcessProperty.NAME) {
            return `${defaultPropertyLabel}${currentOrdering.rev ? "▲" : "▼"}`;
        } else {
            return `${currentOrdering.rev ? "▲" : "▼"}${defaultPropertyLabel}`;
        }
    };

    return (
        <div className="stats-container flex-column">
            <div>{props.procs.length} active processes</div>
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
                                pidLabelDefault
                            )}
                        </span>
                    </div>
                    <div></div>
                    {props.procs.map((p) => (
                        <div className="col-content">{p.pid}</div>
                    ))}
                </div>
                <div className="name-col proc-col d-flex flex-column">
                    <div
                        className="text-nowrap"
                        onClick={() => sortProcessesBy(ProcessProperty.NAME)}
                    >
                        <span
                            className={`col-content${
                                ordering.ord === ProcessProperty.NAME
                                    ? " text-decoration-underline"
                                    : ""
                            }`}
                        >
                            {propertyLabel(
                                ProcessProperty.NAME,
                                ordering,
                                nameLabelDefault
                            )}
                        </span>
                    </div>
                    <div></div>
                    {props.procs.map((p) => (
                        <div className="col-content">{p.name}</div>
                    ))}
                </div>
                <div className="threads-col proc-col d-flex flex-column">
                    <div
                        className="text-nowrap threads-label"
                        onClick={() => sortProcessesBy(ProcessProperty.THREADS)}
                    >
                        <span
                            className={`col-content${
                                ordering.ord === ProcessProperty.THREADS
                                    ? " text-decoration-underline"
                                    : ""
                            }`}
                        >
                            {propertyLabel(
                                ProcessProperty.THREADS,
                                ordering,
                                threadsLabelDefault
                            )}
                        </span>
                    </div>
                    <div></div>
                    {props.procs.map((p) => (
                        <div className="col-content">{p.threads}</div>
                    ))}
                </div>
                <div className="memory-col proc-col d-flex flex-column">
                    <div
                        className="text-nowrap mem-label"
                        onClick={() => sortProcessesBy(ProcessProperty.MEM)}
                    >
                        <span
                            className={`col-content${
                                ordering.ord === ProcessProperty.MEM
                                    ? " text-decoration-underline"
                                    : ""
                            }`}
                        >
                            {propertyLabel(
                                ProcessProperty.MEM,
                                ordering,
                                memLabelDefault
                            )}
                        </span>
                    </div>
                    <div></div>
                    {props.procs.map((p) => (
                        <div className="col-content">
                            {formatBytes(p.mem, {
                                short: true,
                                space: false,
                                roundTreshold: 10,
                                absoluteRoundTreshold: 1024 ** 3,
                                roundingDigits: 1,
                            })}
                        </div>
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
                                cpuLabelDefault
                            )}
                        </span>
                    </div>
                    <div></div>
                    {props.procs.map((p) => (
                        <div className="col-content cpu-percs">
                            {((p.cpu_usage / currentTotal) * 100)
                                .toFixed(1)
                                .padStart(5, " ")}
                            %
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
}
