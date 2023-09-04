const padHex = (s: string) => s.padStart(2, "0");

export const getHighHex = (v: number) => {
  const hex = v.toString(16).toUpperCase();
  return "0x" + padHex(hex.slice(0, 2));
};

export const getLowHex = (v: number) => {
  const hex = v.toString(16).toUpperCase();
  return "0x" + padHex(hex.slice(2, 4));
};
