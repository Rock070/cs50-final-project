import type { PageServerLoad } from "./$types.js";
import { superValidate } from "sveltekit-superforms";
import { urlFormSchema } from './schema';
import { zod } from "sveltekit-superforms/adapters";
 
export const load: PageServerLoad = async () => {
  const form = await superValidate(zod(urlFormSchema));

  return {
    form,
 };
};