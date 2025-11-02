import re
import requests
from bs4 import BeautifulSoup
import time
from typing import Dict, List
from collections import defaultdict

def fetch_and_process_stamps(url):
    print(f"Attempting to fetch data from: {url}")
    
    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
    }
    
    try:
        print("Sending request...")
        response = requests.get(url, headers=headers, timeout=10)
        print(f"Response status code: {response.status_code}")
        
        if response.status_code != 200:
            print(f"Error: Received status code {response.status_code}")
            return
        
        print("Parsing HTML content...")
        soup = BeautifulSoup(response.text, 'html.parser')
        
        # Debug: Print the first part of the HTML to see what we're getting
        print("\nFirst 500 characters of HTML:")
        print(response.text[:500])
        print("\nNumber of <span> tags found:", len(soup.find_all('span')))
        
        # Debug: Print first few spans
        print("\nFirst 3 spans found:")
        for span in list(soup.find_all('span'))[:3]:
            print(f"SPAN TEXT: {span.get_text().strip()}")
        
        print("\nProcessing stamps...\n")
        
        # Regular expressions for filtering
        year_pattern = r'\s18\d{2}.|\s19[0-3]\d\s.|\s19[0-3]\d-[0-3]\d.|19\d{2}-40.|1940\s.'
        auction_ref_pattern = r'(?:^|\s)(\d{1,4})\.(?=\s)'
        
        exclude_keywords = [
            "jpg", "CATALOGUE", "BLOCK", "CANADA", "Booklets", 
            "SPECIMEN", "cover", "Souvenir", "CINDERELLAS",
            "BANK NOTE", "POSTCARDS", "COVERS"
        ]
        
        # Dictionary to store results by country
        results: Dict[str, List[tuple]] = defaultdict(list)
        
        spans = soup.find_all('span')
        
        for span in spans:
            text = span.get_text().strip()
            
            # Debug: Print each text we're processing
            print(f"Processing text: {text[:100]}")
            
            auction_ref = None
            ref_match = re.search(auction_ref_pattern, text)
            if ref_match:
                auction_ref = ref_match.group(1)
                print(f"Found auction ref: {auction_ref}")
            
            if (re.search(year_pattern, text) and 
                not any(keyword in text for keyword in exclude_keywords)):
                
                country_match = re.match(r'^[A-Z]+(?: [A-Z]+)?', text)
                if country_match and auction_ref:
                    country = country_match.group(0)
                    stamp_numbers = re.findall(r'#\w+', text)
                    print(f"Found match - Country: {country}, Stamps: {stamp_numbers}")
                    
                    for number in stamp_numbers:
                        stamp_num = number.replace('#', '')
                        results[country].append((auction_ref, stamp_num, text.strip()))

        # Print results in a clean format
        if results:
            print("\n" + "="*80)
            print("STAMP AUCTION RESULTS")
            print("="*80 + "\n")
            
            for country in sorted(results.keys()):
                print(f"\n{country}:")
                print("-" * len(country) + "-\n")
                
                for auction_ref, stamp_num, description in sorted(results[country], key=lambda x: x[0]):
                    print(f"Lot #{auction_ref:4} | Stamp #{stamp_num:8} | {description[:100]}...")
                
            print("\n" + "="*80)
            print(f"Total countries: {len(results)}")
            print(f"Total stamps found: {sum(len(stamps) for stamps in results.values())}")
            print("="*80 + "\n")
        else:
            print("\nNo matching stamps found.")

    except requests.exceptions.Timeout:
        print("Error: The request timed out")
    except requests.exceptions.ConnectionError:
        print("Error: Failed to connect to the website")
    except Exception as e:
        print(f"An unexpected error occurred: {str(e)}")

def main():
    url = "http://www.fvhstamps.com/WeeklyAuctions/FvhWA.htm"
    print("Starting stamp scraper...")
    start_time = time.time()
    
    try:
        fetch_and_process_stamps(url)
    except Exception as e:
        print(f"Fatal error: {e}")
    
    end_time = time.time()
    print(f"Script completed in {end_time - start_time:.2f} seconds")

if __name__ == "__main__":
    main() 