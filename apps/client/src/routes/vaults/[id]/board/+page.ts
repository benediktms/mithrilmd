export function load({ params }) {
  const id = parseInt(params.id);
  return { vaultId: id };
}
