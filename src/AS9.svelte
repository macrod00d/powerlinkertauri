<script>
    // You can define your data and logic here
    let searchInput = "";
  </script>
  
  <style>
    .dashboard {
      display: grid;
      grid-template-columns: repeat(4, 1fr);
      gap: 20px;
      padding: 20px;
      background-color: #2c3e50;
      font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    }
  
    .panel {
      background-color: #34495e;
      border-radius: 4px;
      padding: 20px;
      box-shadow: 0px 0px 10px rgba(0, 0, 0, 0.1);
    }
  
    .panel h2 {
      color: #ecf0f1;
      margin-bottom: 10px;
    }
  
    .button {
      display: block;
      padding: 10px;
      margin-bottom: 5px;
      background-color: #3498db;
      color: white;
      text-align: left;
      border: none;
      border-radius: 4px;
      cursor: pointer;
      transition: background-color 0.3s;
      text-decoration: none;
    }
  
    .button:hover {
      background-color: #2980b9;
    }
  
    .search-box {
      grid-column: 1 / span 4;
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 10px;
      margin-bottom: 20px;
      background-color: #34495e;
      border-radius: 4px;
    }
  
    .search-box input {
      width: 100%;
      padding: 10px;
      margin-right: 10px;
      border: none;
      border-radius: 4px;
      font-size: 16px;
    }
  
    .search-box button {
      padding: 10px 20px;
      background-color: #3498db;
      color: white;
      border: none;
      border-radius: 4px;
      cursor: pointer;
      transition: background-color 0.3s;
    }
  
    .search-box button:hover {
      background-color: #2980b9;
    }
  
    .week {
      color: #ecf0f1;
      font-size: 1.5rem;
      padding: 0 20px;
    }
  
    .week-box {
      display: flex;
      align-items: center;
    }
  </style>
  
  <div class="dashboard">
    <div class="search-box">
      <input type="text" bind:value={searchInput} placeholder="Type or Paste here" />
      <button>Search</button>
    </div>
  
    <div class="panel">
      <h2>AS7 PANELS</h2>
      <button class="button">Action Request</button>
      <button class="button">Assignment Tracking</button>
      <!-- Add the rest of the buttons from the mockup here -->
    </div>
  
    <div class="panel">
      <h2>WORK ORDERS</h2>
      <button class="button">Asset Suite</button>
      <button class="button">AAA Summary</button>
      <!-- Add the rest of the buttons from the mockup here -->
    </div>
  
    <div class="panel">
      <h2>OTHER</h2>
      <button class="button">Acronyms</button>
      <button class="button">CATID-SCWMT</button>
      <!-- Add the rest of the buttons from the mockup here -->
    </div>
  
    <div class="panel">
      <h2>TRAINING</h2>
      <button class="button">My Scheduled Training</button>
      <button class="button">My Pending Training</button>
      <!-- Add the rest of the buttons from the mockup here -->
    </div>
  
    <!-- Repeat for the remaining categories and buttons as seen in the mockup -->
  
    <div class="week-box">
      <div class="week">WEEK 45</div>
    </div>
  </div>
