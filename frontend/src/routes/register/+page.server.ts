import type { PageServerLoad } from "../login/$types.js";
import { superValidate } from "sveltekit-superforms";
import { registerFormSchema } from '../schema.js';
import { zod } from "sveltekit-superforms/adapters";
 
export const load: PageServerLoad = async () => {
  const form = await superValidate(zod(registerFormSchema));

  return {
    form,
 };
};