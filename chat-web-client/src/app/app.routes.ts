import {mapToCanActivate, Routes} from '@angular/router';
import {LoginComponent} from "./component/login/login.component";
import {PublicChatComponent} from "./component/public-chat/public-chat.component";
import {PublicChatGuard} from "./authguards/public-chat-guard.service";

export const routes: Routes = [
    { path: '', component: LoginComponent },
    { path: 'public-chat', canActivate: mapToCanActivate([PublicChatGuard]), component: PublicChatComponent },
    { path: '**', redirectTo: '' }
];
