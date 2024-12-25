export type HashUrlResponse = {
  /**
   * 縮短後的 url
   */
  url: string;
}


export type UserResponse = {
  /**
   * 使用者 email
   */
  email: string

  /**
   * 使用者 id
   */
  id: string

  /**
   * 使用者名稱
   */
  username: string
}


export type LoginResponse = UserResponse & {
  /**
   * 使用者 token
   */
  token: string
}

export type RegisterResponse = UserResponse

export type UserUrl = {
  id: string
  /**
   * 縮短後的 url
   */
  short_url: string

  /**
   * 原始 url
   */
  url: string

  /**
   * 請求次數
   */
  request_count: number

  /**
   * 建立時間
   */
  created_at: string
}

export type UserUrlsResponse = {
  /**
   * 使用者 url
   */
  urls: UserUrl[]
}

