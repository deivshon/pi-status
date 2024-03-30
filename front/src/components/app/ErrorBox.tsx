type ErrorBoxProps = {
    error: string;
};

export const ErrorBox = (props: ErrorBoxProps) => {
    return (
        <div className="flex h-full items-center justify-center text-sm">
            <div className="no-scrollbar h-[50vh] w-[50vw] items-center justify-center overflow-scroll rounded-xl bg-red-400 px-4 py-2">
                <p className="text-center text-red-950">{props.error}</p>
            </div>
        </div>
    );
};
