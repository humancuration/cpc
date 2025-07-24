import { GraphQLClient } from '../graphql/client';
import { showToast } from '../stores/toast';
import { IncomingWebhook } from '@slack/webhook';
import analytics from '$lib/analytics';

const SLACK_WEBHOOK_URL = process.env.SLACK_BETA_FEEDBACK_WEBHOOK;

export default class NotificationService {
  constructor() {
    this.client = new GraphQLClient();
  }

  async sendNotification(userIds, title, message, category) {
    try {
      // In a real implementation, this would call a GraphQL mutation
      console.log(`Sending notification to users: ${userIds.join(', ')}`);
      console.log(`Title: ${title}, Message: ${message}, Category: ${category}`);
      
      // For demo purposes, show a toast notification
      showToast.set({
        message: `Notification sent: ${title}`,
        type: 'info'
      });

      // Send to Slack if it's a feedback notification
      if (category === 'feedback') {
        await this.sendSlackNotification(`New Feedback: ${title} - ${message}`);
      }
      
      return true;
    } catch (error) {
      console.error('Failed to send notification:', error);
      showToast.set({
        message: `Failed to send notification: ${error.message}`,
        type: 'error'
      });
      return false;
    }
  }

  async sendSlackNotification(message) {
    if (!SLACK_WEBHOOK_URL) {
      console.warn('SLACK_WEBHOOK_URL not set, skipping Slack notification');
      return;
    }
    
    try {
      const webhook = new IncomingWebhook(SLACK_WEBHOOK_URL);
      await webhook.send({
        text: message,
        channel: '#beta-feedback'
      });
    } catch (err) {
      console.error('Slack notification failed', err);
    }
  }

  async sendWeeklySummary() {
    if (!SLACK_WEBHOOK_URL) {
      console.warn('SLACK_WEBHOOK_URL not set, skipping weekly summary');
      return;
    }
    
    try {
      // In a real implementation, this would fetch actual summary data
      const summary = "Weekly summary placeholder - would include stats on feedback, usage, etc.";
      
      const webhook = new IncomingWebhook(SLACK_WEBHOOK_URL);
      await webhook.send({
        text: `*Weekly Beta Summary*\n${summary}`,
        channel: '#beta-feedback',
        mrkdwn: true
      });
      
      analytics.track('training_scheduled', {
        training_type: 'weekly_summary',
        status: 'sent'
      });
    } catch (err) {
      console.error('Failed to send weekly summary', err);
      analytics.track('training_scheduled', {
        training_type: 'weekly_summary',
        status: 'failed',
        error: err.message
      });
    }
  }

  async scheduleReminder(event, minutesBefore) {
    const reminderTime = new Date(event.start_time);
    reminderTime.setMinutes(reminderTime.getMinutes() - minutesBefore);
    
    const now = new Date();
    const delay = reminderTime - now;
    
    if (delay > 0) {
      setTimeout(() => {
        this.sendNotification(
          event.attendees,
          `Reminder: ${event.title} starting soon`,
          `Your session starts in ${minutesBefore} minutes`,
          'session_reminder'
        );
      }, delay);
    }
  }

  async notifyScheduleChange(event, oldTime) {
    this.sendNotification(
      event.attendees,
      `Schedule Change: ${event.title}`,
      `Session time changed from ${oldTime} to ${event.start_time}`,
      'schedule_change'
    );
  }

  async sendFollowUp(event, materials) {
    this.sendNotification(
      event.attendees,
      `Follow-up materials for ${event.title}`,
      `Materials: ${materials.join(', ')}`,
      'follow_up'
    );
  }
}