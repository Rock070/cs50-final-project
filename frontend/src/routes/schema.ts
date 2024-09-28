import { z } from 'zod';

// 登入 form schema
export const loginFormSchema = z.object({
  username: z.string().min(1),
  password: z.string().min(1),
});

// 註冊 form schema
export const registerFormSchema = loginFormSchema.extend({
  name: z.string().min(1),
});
export const urlFormSchema = z.object({
  url: z.string().url(),
});

export type UrlFormSchema = typeof urlFormSchema;
export type LoginFormSchema = typeof loginFormSchema;
export type RegisterFormSchema = typeof registerFormSchema;