
import { redirect } from '@sveltejs/kit';
import { COOKIE_KEY } from '$lib/constant';
import type { UserUrlsResponse } from '$lib/type/api';
import request from '$lib/request/core';
import type { UserUrl } from '$lib/type/api';

export async function load({ cookies }) {
  const token = cookies.get(COOKIE_KEY.TOKEN) || '';

  if(!token) {
    throw redirect(302, '/login')
  }

  let urls: UserUrl[] = [];

  await request<UserUrlsResponse>('/user/urls', { token })
    .then (res => {
      urls = res.data.urls;
    })
    .catch(err => {
      console.error(err);
    });

  return {
    token,
    urls
  };
}