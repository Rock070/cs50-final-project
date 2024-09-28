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

export type RegisterResponse = LoginResponse