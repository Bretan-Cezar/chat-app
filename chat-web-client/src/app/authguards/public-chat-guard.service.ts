import { Injectable } from '@angular/core';
import {PublicUserSocketService} from "../service/public-user-socket.service";
import {Router} from "@angular/router";

@Injectable({
  providedIn: 'root'
})
export class PublicChatGuard {

  constructor(private socketService: PublicUserSocketService,
              private router: Router) { }

  canActivate() {

    if (this.socketService.isSocketOpen) {
      return true
    }
    else {
      this.router.navigate(["../"])
      return false
    }
  }
}
