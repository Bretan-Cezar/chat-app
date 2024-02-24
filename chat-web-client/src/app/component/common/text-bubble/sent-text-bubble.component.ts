import {Component, Input} from '@angular/core';
import { CommonModule } from '@angular/common';
import {TextMessage} from "../../../shared/data-type/textMessage";
import {getTimeFromISO8601String} from "../../../shared/utils/DateTimeConversion";

@Component({
  selector: 'app-sent-text-bubble',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './sent-text-bubble.component.html',
  styleUrl: './sent-text-bubble.component.scss'
})
export class SentTextBubbleComponent {

  @Input() textMessage: TextMessage | null = null
  protected readonly getTimeFromISO8601String = getTimeFromISO8601String;
}
