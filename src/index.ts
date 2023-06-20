import { Client } from '@newtykins/botkit';
import dotenv from 'dotenv';

dotenv.config();

const client = new Client(process.env.TOKEN as string);

client.login();
