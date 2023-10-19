const padHex = (s: string) => s.padStart(2, "0");

export const getHighHex = (v: number) => {
  const highBits = 0xFF00 & v;
  const hex = highBits.toString(16).toUpperCase();
  return "0x" + padHex(hex.slice(0, 2));
};

export const getLowHex = (v: number) => {
  const lowBits = 0x00FF & v;
  const hex = lowBits.toString(16).toUpperCase();
  return "0x" + padHex(hex);
};

