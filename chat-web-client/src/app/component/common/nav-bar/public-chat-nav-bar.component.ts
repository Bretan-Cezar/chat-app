import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import {MatButtonModule} from "@angular/material/button";
import {MatIconModule} from "@angular/material/icon";
import {PublicUserSocketService} from "../../../service/public-user-socket.service";
import {Router} from "@angular/router";

@Component({
  selector: 'app-public-chat-nav-bar',
  standalone: true,
  imports: [CommonModule, MatButtonModule, MatIconModule],
  templateUrl: './public-chat-nav-bar.component.html',
  styleUrl: './public-chat-nav-bar.component.scss'
})
export class PublicChatNavBarComponent {

  constructor(
      private socketService: PublicUserSocketService,
      private router: Router
  ) { }

  logout() {

    this.socketService.logoutPublicUser()
    this.router.navigate(['../'])
  }
}
