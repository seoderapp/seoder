export interface Props {
  id: string;
  name?: string;
  onChange?(x: any): any;
  checked?: boolean;
}

export const Switch = ({ id, name, onChange, checked }: Props) => {
  return (
    <label className="switch">
      <input
        name={name}
        id={id}
        checked={checked}
        type="checkbox"
        role="switch"
        onChange={onChange}
      />
      <span className="slider"></span>
    </label>
  );
};
