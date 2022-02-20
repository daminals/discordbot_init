# bot.py
# Daniel Kogan
# 02.20.2022

from discord.ext import commands
from dotenv import load_dotenv
import discord, os

intents = discord.Intents.all()
load_dotenv()
TOKEN = os.environ.get('TOKEN', 3)

bot = commands.Bot(command_prefix="?", intents=intents)

@bot.event
async def on_ready():
    print(f"We have logged in as {bot.user}")

@bot.command()
async def ping(ctx):
    await ctx.send("pong")
    
bot.run(TOKEN)