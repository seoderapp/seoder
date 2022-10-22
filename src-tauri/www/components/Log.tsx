export interface Props {
  id: string;
  emptyId: string;
}

export function Log({ id, emptyId }: Props) {
  return (
    <div className="log">
      <ul id={id}></ul>
      <div id={emptyId} className={"hidden"}>
        Ready to run
      </div>
    </div>
  );
}
