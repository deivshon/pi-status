type ErrorBoxProps = {
    error: string;
};

export const ErrorBox = (props: ErrorBoxProps) => {
    return (
        <div
            className="h-100 container d-flex justify-content-center align-items-center"
            id="error-box"
        >
            <div className="col-md-6">
                <div className="alert alert-danger text-center" role="alert">
                    {props.error}
                </div>
            </div>
        </div>
    );
};
