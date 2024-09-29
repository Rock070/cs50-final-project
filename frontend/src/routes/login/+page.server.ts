import type { PageServerLoad } from "./$types.js";
import { superValidate } from "sveltekit-superforms";
import { loginFormSchema } from '../schema';
import { zod } from "sveltekit-superforms/adapters";
 
export const load: PageServerLoad = async () => {
  const form = await superValidate(zod(loginFormSchema));

  return {
    form,
 };
};