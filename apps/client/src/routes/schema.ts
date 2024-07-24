import { z } from 'zod';

export const createVault = z.object({
  name: z.string().min(1, 'Please choose a name for your vault'),
  location: z.string().min(2, 'Please choose a valide path')
});

export type CreateVaultSchema = z.infer<typeof createVault>;
