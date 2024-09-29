import type { LayoutServerLoad } from "./$types.js";
import { COOKIE_KEY } from '$lib/constant';
 
export const load: LayoutServerLoad = async ({ cookies }) => {
  const token = cookies.get(COOKIE_KEY.TOKEN);

  return {
    token,
 };
};