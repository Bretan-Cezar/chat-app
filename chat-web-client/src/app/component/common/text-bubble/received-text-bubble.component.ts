import {Component, Input} from '@angular/core';
import { CommonModule } from '@angular/common';
import {TextMessage} from "../../../shared/data-type/textMessage";
import {
  getTimeFromISO8601String,
  getTimestampFromISO8601String,
  offsetFromUTCISO8601String
} from "../../../shared/utils/DateTimeConversion";

@Component({
  selector: 'app-received-text-bubble',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './received-text-bubble.component.html',
  styleUrl: './received-text-bubble.component.scss'
})
export class ReceivedTextBubbleComponent {

  @Input() textMessage: TextMessage | null = null

  protected readonly getTimeFromISO8601String = getTimeFromISO8601String;
  protected readonly offsetISO8601String = offsetFromUTCISO8601String;
}
